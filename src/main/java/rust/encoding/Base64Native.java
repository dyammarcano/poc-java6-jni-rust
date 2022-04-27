package rust.encoding;

public class Base64Native {

    /**
     * Decodes a Base64 encoded String into a byte array using rust native implementation.
     *
     * @param input The Base64 encoded String to decode.
     * @return The decoded byte array.
     */
    public static native byte[] decode(String input);

    /**
     * Encodes a byte array into a Base64 encoded String using rust native implementation.
     *
     * @param input The byte array to encode.
     * @return The Base64 encoded String.
     */
    public static native String encode(byte[] input);

    /*static {
        System.loadLibrary("rust-encoding");
    }*/
}
