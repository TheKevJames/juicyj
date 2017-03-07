public class Main {
    public Main() {}
    // XXX: we added public to avoid testing "missing access control"
    public static int sf = 0;

    public static int test() {
	sf = 4;
	return 123;
    }
}
