<div align="center">

# 🔍 searchcode-cli

</div>


<div align="center">

A command line client for searchcode.com to find source code on multiple services 

</div>

## About

This tool is a command line client for searchcode.com, a service that allows you to search source code from multiple services at once.
For more information about the API, see: [https://searchcode.com/api/](https://searchcode.com/api/).

<div align="center">
  
![ScreenShot](https://private-user-images.githubusercontent.com/62412884/408856198-583f03a7-54dd-4e08-93ea-6a42ae0ea7e6.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3Mzg0OTg3OTUsIm5iZiI6MTczODQ5ODQ5NSwicGF0aCI6Ii82MjQxMjg4NC80MDg4NTYxOTgtNTgzZjAzYTctNTRkZC00ZTA4LTkzZWEtNmE0MmFlMGVhN2U2LnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNTAyMDIlMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjUwMjAyVDEyMTQ1NVomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPWY4ZjlhMDY5MTJkNjI4Mzk0MzQyYzM0MzZhMjZmZjg1NWM5ZjYzZmVlNDliNDBhMWNiNGViZDZmNjczNDY4ODMmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.JBmix9NbbGPDlJU3Sua_nNO1V64TqueL5mXkFBJM5a0)

</div>

## Usage

```
$ searchcode-cli --help

Usage: searchcode-cli [OPTIONS] <QUERY>

Arguments:
  <QUERY>  

Options:
  -l, --language-code <LANGUAGE_CODE>  
  -p, --page <PAGE>                    
  -m, --max-pages <MAX_PAGES>          
      --min-lines <MIN_LINES>          
      --max-lines <MAX_LINES>          
      --provider-code <PROVIDER_CODE>  
  -h, --help                           Print help
```

## Installation

```sh
git clone https://github.com/sheepla/searchcode-cli.git
cd searchcode-cli
cargo install --path=.
```

## License

MIT

