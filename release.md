# Create a release

- Update to the given version:
  - meson.build
  - cargo.toml
- create a tag on gitlab
- meson builddir -Dbuildtype=release
- ninja -C buiddir dist
- upload the package and the sha256 to gitlab for Flatpak in the release notes

# flathub

https://github.com/flathub/org.freedesktop.dabrain34.GstPipelineStudio

- Need to update the package and the sha256 generated by ninja -C builddir dist
