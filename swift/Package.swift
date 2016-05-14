import PackageDescription

let package = Package(
  name: "helloworld",
  dependencies: [
    .Package(url: "https://github.com/kylef/Curassow.git", majorVersion: 0, minor: 2),
  ]
)
