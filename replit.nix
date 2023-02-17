{ pkgs }: {
	deps = [
		pkgs.openssh_with_kerberos
  pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}