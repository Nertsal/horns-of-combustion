set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

run *OPTIONS:
  cargo run -F dynamic-linking {{OPTIONS}}
