use winres;

fn main() {
  if cfg!(target_os = "windows") {
    let mut res = winres::WindowsResource::new();
    res
      .set_icon("./images/openjdk.ico")
      .set("CompanyName", "toddnguyen47")
      .set("FileDescription", "Java JAR Binary")
      .set("InternalName", "javajar.exe")
      .set("ProductName", "JavaJar");
    res.compile().unwrap();
  }
}
