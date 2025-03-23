{pkgs, ...}: {
  packages = with pkgs; [
    alsa-lib-with-plugins
    udev
    wayland
  ];

  env.LD_LIBRARY_PATH = with pkgs;
    lib.makeLibraryPath [
      libxkbcommon
      vulkan-loader
    ];

  scripts.crr.exec = ''
    cargo clippy && cargo run
  '';

  languages.rust = {
    enable = true;
    channel = "stable";
  };
}
