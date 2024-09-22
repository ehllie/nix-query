{
  perSystem.nci = {
    toolchainConfig = ./rust-toolchain.toml;
    projects.nix-query.path = ./.;
  };
}
