/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use signal_hook::{consts::{SIGHUP,SIGINT,SIGQUIT,SIGTERM}};
use std::{time::Duration};
use std::fs::File;
use daemonize::Daemonize;
use signal_hook::iterator::Signals;
use subprocess::Exec;
use clap::{Arg, App};

pub fn process_siganls() {

    let signals = Signals::new(&[SIGHUP,SIGINT,SIGQUIT,SIGTERM]);

    match signals {
       Ok(mut info)=> {
      //   std::thread::spawn(move ||
           for sig in info.forever() {
              println!("Received signal {:?}", sig);
              match sig {
                SIGHUP | SIGINT | SIGQUIT | SIGTERM => {std::process::exit(sig)},
                _ => {std::thread::sleep(Duration::from_secs(2))}
              }
           }
      //   );
       },
       Err(_) => {std::process::exit(1)},
    };
}

pub fn process_command( cmd: String )  {
     std::thread::spawn(move||{
         loop {
             let mut task = Exec::shell(cmd.clone()).popen().unwrap();
             let status = task.wait();
             match status {
                Ok(val) => {
                    println!("Done with {:?}",val);
                },
                Err(e) => {
                    println!("Failed with {:?}",e);
                    break;
                }
             }
         }
     });
}

pub fn process_daomon_command( cmd: String )  {
    let pid = std::process::id();
    let working_directory = format!("/tmp/processkeeper/{}",pid);
    match std::fs::create_dir_all(working_directory.clone()) {
        Ok(_) => {}
        Err(_) => {return}
    }
    let stdout = File::create(format!("/tmp/processkeeper/{}/stdout.log",pid)).unwrap();
    let stderr = File::create(format!("/tmp/processkeeper/{}/stderr.log",pid)).unwrap();
    let pidfile = format!("/tmp/processkeeper/{}/pidfile",pid);
    let daemonize = Daemonize::new()
        .pid_file(pidfile)
        .chown_pid_file(true)
        .working_directory(working_directory.clone())
        .user("nobody")
        .group("daemon")
        .group(2)
        .umask(0o777)
        .stdout(stdout)
        .stderr(stderr)
        .privileged_action(|| {
            println!("Success, privileged_action")
        }
        );

    match daemonize.start() {
        Ok(_) => {
            println!("Success, daemonized")
        },
        Err(e) => {
            eprintln!("Error, {}", e)
        },
    }

    loop {
        let mut task = Exec::shell(cmd.clone()).popen().unwrap();
        let status = task.wait();
        match status {
            Ok(val) => {
                println!("Done with {:?}",val);
            },
            Err(e) => {
                println!("Failed with {:?}",e);
                break;
            }
        }
    }
}

fn main() {
    //process arguments
    let matches = App::new("Process Keeper")
                  .version("1.0")
                  .author("Brian Gao. <gao.brian@gmail.com>")
                  .about("Tiny Process Keeper")
                  .arg(Arg::with_name("command")
                        .help("COMMAND to run as daemon")
                        .required(true)
                        .index(1))
                  .arg(Arg::with_name("daomon")
                        .short("d")
                        .long("daomon")
                        .takes_value(false)
                        .required(false)
                        .help("Running as daomon process")
                    )
                  .get_matches();

    let isdaemon = matches.is_present("daomon");
    let cmd = matches.value_of("command").unwrap();

    if isdaemon {
        process_daomon_command(String::from(cmd));
    } else {
        process_command(String::from(cmd));
        //handle signal
        process_siganls();
    }
}

