<div align="center">

# 🔍 searchcode-cli

</div>


<div align="center">

A command line client for searchcode.com to find source code on multiple services 

</div>

## About

This tool is a command line client for searchcode.com, a service that allows you to search source code from multiple services at once.
For more information about the API, see: [https://searchcode.com/api/](https://searchcode.com/api/).


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

