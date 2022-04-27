package rust.encoding;

import org.junit.Test;

import java.io.File;

import static org.junit.Assert.*;

public class Base64NativeTest {

    static {
        File f = new File("D:\\Proof of Concept\\Java\\Experimental\\JavaRustFFI\\rust\\library\\target\\debug\\jni_library.dll");
        System.load(f.getAbsolutePath());
    }

    @Test
    public void decode() {

        String result = Base64Native.encode("hello world".getBytes());
        assertEquals("aGVsbG8gd29ybGQ=", result);
    }

    @Test
    public void encode() {

        byte[] result = Base64Native.decode("aGVsbG8gd29ybGQ=");
        assertArrayEquals("hello world".getBytes(), result);
    }
}