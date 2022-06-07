![Quo](assets/quo-client-trans.png)

<p align="center">
    <a href="https://github.com/protoqol/quo/actions/workflows/main.yml">	
       <img alt="Github action status" src="https://github.com/protoqol/quo/actions/workflows/main.yml/badge.svg">
    </a>        
    <a href="https://github.com/protoqol/quo/actions/workflows/release.yml">	
       <img alt="Release" src="https://github.com/protoqol/quo/actions/workflows/release.yml/badge.svg">
    </a>    
    <a href="https://twitter.com/intent/follow?screen_name=Protoqol_XYZ">
        <img src="https://img.shields.io/twitter/follow/Protoqol_XYZ.svg?label=%40Protoqol_XYZ&style=social"
            alt="Follow Protoqol on Twitter">
    </a>
</p>

# Quo

> Quo is currently in beta, releases are only available as zip unless you built Quo yourself.
> Quo should __not__ be used in production environments.


[Quo is a free, open-source, client-side debugger and can be found here](https://github.com/Protoqol/Quo).

Quo is a debugging companion to help you debug dumped variables, the dumped variables will appear in this Quo client
instead of the
traditional way which is often tedious.

### Companion packages

- [Quo-PHP](https://github.com/Protoqol/Quo-php)

## Features

- See which variable was dumped (name or value)
- Nested variables unfolding (thanks to symfony/var-dumper)

![Quo](assets/preview.png)

- Search dumped variables

![Quo](assets/preview_search.png)

## Install - only zip available for now.

1. Head over to [the releases](https://github.com/Protoqol/Quo/releases)
2. Look for the latest version
3. Download the zip for your OS (quo-darwin-\*.zip for macOS, quo-linux-\*.zip for Linux, quo-win32-\*.zip for windows)
4. Extract wherever, and run the Quo executable.

- You need a companion package like [Quo-php](https://github.com/Protoqol/Quo-php) to send variables to Quo.
- Default (unchangeable for now) hostname and port for Quo are 127.0.0.1:7312.

## Issues

#### Issues, bugs and feature requests can be reported [here!](https://github.com/Protoqol/quo-php/issues/new/choose)

## Contributing

See [Contributing](CONTRIBUTING.md) to see how you can contribute to Quo!

## Contributors

- [Quinten Justus](https://github.com/QuintenJustus)
- [Contributors](https://github.com/Protoqol/quo-php/graphs/contributors)

## License

Quo is licensed under the MIT License. Please see [License File](LICENSE) for more information.
