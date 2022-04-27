package rust.encoding;

import org.junit.Test;

import java.io.File;

import static org.junit.Assert.*;

public class Base58NativeTest {

    static {
        File f = new File("D:\\Proof of Concept\\Java\\Experimental\\JavaRustFFI\\rust\\library\\target\\debug\\jni_library.dll");
        System.load(f.getAbsolutePath());
    }

    @Test
    public void decode() {

        String result = Base58Native.encode("hello world".getBytes());
        assertEquals("StV1DL6CwTryKyV", result);
    }

    @Test
    public void encode() {

        byte[] result = Base58Native.decode("StV1DL6CwTryKyV");
        assertArrayEquals("hello world".getBytes(), result);
    }
}