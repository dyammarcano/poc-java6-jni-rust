package com.rustlang.jni;

import java.net.URISyntaxException;


/**
 * HttpClient class to overcome the problem of the java 1.6.0_31 version referent to TLSv1.2 and TLSv1.3.
 *
 * @author <a href="mailto:dyam.daniels@vilt-group.cpm">Dyam Daniels</a>
 * @since 1.0.0
 * @version 1.0.0
 */
public class HttpsClientReqwest {

    public static HttpsClientReqwest getInstance() {
        return new HttpsClientReqwest();
    }

    /**
     * (reqwest::blocking::get) library to be implemented by the native library using JNI and Rust Language.
     *
     * @param url the url to connect to
     * @return the response body
     * @throws URISyntaxException if the url is invalid
     */
    public native String httpGet(String url) throws URISyntaxException;

    /**
     * (reqwest::blocking::post) library to be implemented by the native library using JNI and Rust Language.
     *
     * @param url the url to connect to
     * @param body the body to send
     * @return the response body
     * @throws URISyntaxException if the url is invalid
     */
    public native String httpPost(String url, String body) throws URISyntaxException;
}
