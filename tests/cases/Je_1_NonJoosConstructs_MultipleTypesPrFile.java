// JOOS1:PARSER_WEEDER,PARSER_EXCEPTION
// JOOS2:PARSER_WEEDER,PARSER_EXCEPTION
/**
 * Parser/weeder:
 * - Multiple types pr. file not allowed.
 */
public class Je_1_NonJoosConstructs_MultipleTypesPrFile {

    public Je_1_NonJoosConstructs_MultipleTypesPrFile(){}

    public static int test() {
    	return 123;
    }
}

// XXX: we added the "public" to ensure this test only tests multiple classes
public class A {

    public A(){}
}
