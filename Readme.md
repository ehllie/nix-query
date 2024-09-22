# Nix search TUI

I want to create a TUI application for searching nix module options and packages.
I have the following goals with the project:

- Search through NixOS packages and options, [like](https://search.nixos.org/)
- Have the search results be equivalent to those of the website, but do it all without the need of a network request.
- Allow for easy navigation to (local) package and module source code, and open it with a user configured editor.
- Allow extending the index with module options and packages from flakes, allowing for easier home-manager option search.
- Make it FAST. The modules and packages should only need to be indexed once after each system update.
