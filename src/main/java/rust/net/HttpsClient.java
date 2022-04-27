package rust.net;

import java.net.URL;

public class HttpsClient {

    private URL url;

    private native String connect(URL url);
    private native int getStatus();
}
