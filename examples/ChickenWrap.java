// Setup: Requires a Java SDK and a recent JFFI and JNR-FFI, for instance the ones at:
//   1. https://repo1.maven.org/maven2/com/github/jnr/jnr-ffi/2.1.1/jnr-ffi-2.1.1.jar
//   2. http://central.maven.org/maven2/com/github/jnr/jffi/1.2.13/jffi-1.2.13.jar
//   3. http://forge.ow2.org/project/download.php?group_id=23&file_id=20921
//
// Compile:
//   jar xf jnr-ffi-2.1.1.jar
//   jar xf jffi-1.2.13.jar
//   jar xf asm-5.1.jar
//   ln -s target/release/libchickenize.so libchickenize.so
//   javac examples/ChickenWrap.java
//
// Run:
//   java examples.ChickenWrap
//

package examples;

import java.io.File;

import jnr.ffi.LibraryLoader;

import static java.lang.System.mapLibraryName;

public class ChickenWrap {

  public static String getLibraryPath(String dylib) {
    File f = new File(ChickenWrap.class.getClassLoader().getResource(mapLibraryName(dylib)).getFile());
    return f.getParent();
  }

  public interface LibChickenize {
    String chickenize(String input);
    String buffalo(String input);
    String lorem_ipsum(String input);
    String anonymize(String input, String replacement);
  }

  public static void main(String[] args) {
    System.setProperty("jnr.ffi.library.path", getLibraryPath("chickenize"));

    LibChickenize chicklib = LibraryLoader.create(LibChickenize.class).load("chickenize");

    String example = "The Vice-president didn't like his style";

    String chickenized = chicklib.chickenize(example);
    System.out.println("\nChickenize: \n" + chickenized);

    String buffalo = chicklib.buffalo(example);
    System.out.println("\nChickenize: \n" + buffalo);

    String lorem_ipsum = chicklib.lorem_ipsum(example);
    System.out.println("\nChickenize: \n" + lorem_ipsum);

    String anonymized = chicklib.anonymize(example, "meow");
    System.out.println("\nChickenize: \n" + anonymized);
    System.out.println("");
  }
}
