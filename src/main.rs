use std::env;
use std::process::Command;

#[cfg(target_os = "windows")]
fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() > 1 {
      let var = &args[1];

      if var == "-h" {
          help();
      }

      else if var == "-del" {
          delete(&args[2]);
      }
      else if var == "-dow" {
          download(&args[2], &args[3]);
      }

      else {
        Command::new("powershell")
          // ! Działająca wersja
          // .args(&["/c","Invoke-WebRequest","-Uri",&format!("https://transfer.sh/{var}"),"-Method Put", "-InFile",var]) 
          // ! TESTOWA wersja
          .args(&["/c","Invoke-WebRequest","-Uri","https://transfer.sh/tak.txt","-Method Put", "-InFile",".\\plik.txt"])
          .spawn()
          .expect("Failed to upload file");
      }

  } else {
      println!("Missing argument.");
      println!("Use -h for help.");
  }
}

#[cfg(target_os = "windows")]
fn help(){
  println!("Unofficial Transfer.sh CLI\n");
  println!("Usage: transfer.exe [OPTIONS]\n");
  print!("Options: 
  -h                         Print help
  -dow <link> <file name>    Dowload file
  -del <delete-link>         Delete file");
}

#[cfg(target_os = "windows")]
fn delete(link: &str){
  Command::new("powershell")
    .args(&["/c","Invoke-WebRequest","-Uri",link,"-Method Delete"])
    .spawn()
    .expect("Failed to delete file");
}

#[cfg(target_os = "windows")]
fn download(link: &str, file_name: &str){
  Command::new("powershell")
    .args(&["/c","Invoke-WebRequest","-Uri",link,"-OutFile",file_name])
    .spawn()
    .expect("Failed to download file");
}
