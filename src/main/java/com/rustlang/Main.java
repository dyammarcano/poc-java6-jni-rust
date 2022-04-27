package com.rustlang;

import com.rustlang.jni.HttpsClientReqwest;
import java.io.File;
import java.net.URISyntaxException;


public class Main {

    static HttpsClientReqwest httpsClientReqwest = new HttpsClientReqwest();

    static {
        File f = new File("D:\\Proof of Concept\\Java\\Experimental\\JavaRustFFI\\rust\\library\\target\\debug\\jni_library.dll");
        System.load(f.getAbsolutePath());
        System.out.println("f: " + f.getAbsolutePath());
    }

    public static void main(String[] args) throws URISyntaxException {

        String result = httpsClientReqwest.httpGet("https://reqbin.com/echo");
        System.out.println(result);

        result = httpsClientReqwest.httpPost("https://reqbin.com/echo/post/json", "{\"Id\": 12345,\"Customer\": \"John Smith\",\"Quantity\": 1,\"Price\": 10.00}");
        System.out.println(result);
    }
}
