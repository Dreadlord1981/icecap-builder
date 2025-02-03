use std::{io::Read, net::TcpStream};

use ssh2::Session;

use crate::{cli::{Arguments, BuildType}, Instruction};

pub struct Operation;

impl Operation {

	fn create_lines(&self, args: &Arguments) -> Vec<String> {

		let deploy = &args.deploy;
		let bin = &args.bin;
		let ifs = &args.ifs;

		let build_type = match args.build {
			BuildType::All => {
				"-B allprograms"
			},
		};

		let mut commands = vec![];

		commands.push(format!("PATH=/QOpenSys/pkgs/bin:$PATH && cd {ifs} && gmake {build_type} BIN_LIB={bin}"));
	
		if args.release {

			commands.push(format!("PATH=/QOpenSys/pkgs/bin:$PATH && cd {ifs} && gmake release"));
			commands.push(format!("system -v \"CRTLIB {deploy}\""));
			commands.push(format!("system -v \"CPYFRMSTMF FROMSTMF('{ifs}/release/release.savf') TOMBR('/QSYS.lib/{deploy}.lib/RELEASE.FILE') MBROPT(*REPLACE) CVTDTA(*NONE)\""));
			commands.push(format!("system -v \"RSTOBJ RSTLIB({deploy}) OBJ(*ALL) SAVLIB({bin}) DEV(*SAVF) SAVF({deploy}/RELEASE)\""));
			commands.push(format!("system -v \"CALL {deploy}/CAPSETUP {deploy}\""));
			commands.push(format!("system -v \"CALL ICEPKG/ICBPLUGIN {deploy}\""));
		}
		

		commands
	}

	fn execute_lines(&self, commands: Vec<String>, session: &Session) -> Result<(), anyhow::Error> {

		let mut buffer = [0; 1024];

		for cmd  in commands {

			let mut channel = session.channel_session()?;
			let mut stdin = channel.stream(0);

			channel.exec(&cmd)?;

			let mut len = 0;

			loop {

				let stdin_result = stdin.read(&mut buffer);

				if let Ok(len_result) = stdin_result {

					len = len_result;

					let msg_uft8 = std::str::from_utf8(&buffer[..len]);

					if msg_uft8.is_ok() {

						let message = String::from(msg_uft8?);

						if !message.is_empty() {

							print!("{}", message);
						}
					}
				}

				if len == 0 {
					break;
				}
			}
		}

		Ok(())
	}
}

impl Instruction for Operation {
	fn run(&self, args: crate::cli::Arguments) -> Result<(), anyhow::Error> {
		
		let connection_str = format!("{}:22", &args.host);

		println!("Conneting to: {}", &args.host);

		let tcp = TcpStream::connect(connection_str)?;

		let mut session = Session::new()?;

		session.set_compress(true);
		session.set_tcp_stream(tcp);
	
		session.handshake()?;
		session.userauth_password(&args.user, &args.password)?;

		println!("Connection complete");

		let commands = self.create_lines(&args);

		println!("Operation start");

		self.execute_lines(commands, &session)?;

		println!("Operation Complete");
	

		Ok(())
	}
}

pub struct IcecapOperation;

impl IcecapOperation {
	fn create_lines(&self, _args: &Arguments) -> Vec<String> {
		vec![
			format!("system -v \"CALL PGM(QCMDEXC) PARM('ICEPKG/ICECAPBLD' 16)\"")
		]
	}

	fn execute_lines(&self, commands: Vec<String>, session: &Session) -> Result<(), anyhow::Error> {

		let mut buffer = [0; 1024];
		let mut err_buffer = [0; 1024];

		for cmd  in commands {

			let mut channel = session.channel_session()?;
			let mut stdin = channel.stream(0);
			let mut stderr = channel.stderr();

			channel.exec(&cmd)?;

			let mut len = 0;
			let mut err_len = 0;

			loop {

				let stdin_result = stdin.read(&mut buffer);

				if let Ok(len_result) = stdin_result {

					len = len_result;

					let msg_uft8 = std::str::from_utf8(&buffer[..len]);

					if msg_uft8.is_ok() {

						let message = String::from(msg_uft8?);

						if !message.is_empty() {

							print!("{}", message);
						}
					}
				}

				if let Ok(stderr_len) = stderr.read(&mut err_buffer) {

					err_len = stderr_len;

					let msg_uft8 = std::str::from_utf8(&err_buffer[..stderr_len]);

					if msg_uft8.is_ok() {

						let message = String::from(msg_uft8?);

						if !message.is_empty() {

							print!("{}", message);
						}
					}
				}

				if len == 0 && err_len == 0 {
					break;
				}
			}
		}

		Ok(())
	}
}

impl Instruction for IcecapOperation {
	fn run(&self, args: Arguments) -> Result<(), anyhow::Error> {
		
		let connection_str = format!("{}:22", &args.host);

		println!("Conneting to: {}", &args.host);

		let tcp = TcpStream::connect(connection_str)?;

		let mut session = Session::new()?;

		session.set_compress(true);
		session.set_tcp_stream(tcp);
	
		session.handshake()?;
		session.userauth_password(&args.user, &args.password)?;

		println!("Connection complete");

		let commands = self.create_lines(&args);

		println!("Operation start");

		self.execute_lines(commands, &session)?;

		println!("Operation Complete");
	

		Ok(())
	}
}


pub struct PortfolioOperation;

impl PortfolioOperation {
	fn create_lines(&self, _args: &Arguments) -> Vec<String> {
		vec![
			format!("system -v \"CALL PGM(QCMDEXC) PARM('ICEPKG/PORTFBLD' 15)\"")
		]
	}

	fn execute_lines(&self, commands: Vec<String>, session: &Session) -> Result<(), anyhow::Error> {

		let mut buffer = [0; 1024];
		let mut err_buffer = [0; 1024];

		for cmd  in commands {

			let mut channel = session.channel_session()?;
			let mut stdin = channel.stream(0);
			let mut stderr = channel.stderr();

			channel.exec(&cmd)?;

			let mut len = 0;
			let mut err_len = 0;

			loop {

				let stdin_result = stdin.read(&mut buffer);

				if let Ok(len_result) = stdin_result {

					len = len_result;

					let msg_uft8 = std::str::from_utf8(&buffer[..len]);

					if msg_uft8.is_ok() {

						let message = String::from(msg_uft8?);

						if !message.is_empty() {

							print!("{}", message);
						}
					}
				}

				if let Ok(stderr_len) = stderr.read(&mut err_buffer) {

					err_len = stderr_len;

					let msg_uft8 = std::str::from_utf8(&err_buffer[..stderr_len]);

					if msg_uft8.is_ok() {

						let message = String::from(msg_uft8?);

						if !message.is_empty() {

							print!("{}", message);
						}
					}
				}

				if len == 0 && err_len == 0 {
					break;
				}
			}
		}

		Ok(())
	}
}

impl Instruction for PortfolioOperation {
	fn run(&self, args: Arguments) -> Result<(), anyhow::Error> {
		
		let connection_str = format!("{}:22", &args.host);

		println!("Conneting to: {}", &args.host);

		let tcp = TcpStream::connect(connection_str)?;

		let mut session = Session::new()?;

		session.set_compress(true);
		session.set_tcp_stream(tcp);
	
		session.handshake()?;
		session.userauth_password(&args.user, &args.password)?;

		println!("Connection complete");

		let commands = self.create_lines(&args);

		println!("Operation start");

		self.execute_lines(commands, &session)?;

		println!("Operation Complete");
	

		Ok(())
	}
}