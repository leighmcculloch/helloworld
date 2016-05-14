import static spark.Spark.setPort;
import static spark.Spark.get;

public class HelloWorld {
    static final int PORT_DEFAULT = 4567;

    public static void main(String[] args) {
        String port = System.getenv("PORT");
        if (port != null) {
            setPort(Integer.parseInt(port));
        } else {
            setPort(PORT_DEFAULT);
        }

        get("/", (req, res)-> "Hello world!");
    }
}
