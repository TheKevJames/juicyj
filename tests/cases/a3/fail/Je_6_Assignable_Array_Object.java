// JOOS1:TYPE_CHECKING,ASSIGN_TYPE
// JOOS2:TYPE_CHECKING,ASSIGN_TYPE
// JAVAC:UNKNOWN
//
/**
 * Typecheck:
 * - Type Object is not assignable to type Object[]
 */
public class Je_6_Assignable_Array_Object {

    public Je_6_Assignable_Array_Object () {}

    public static int test() {
        Object[] i = new Object();
        return 123;
    }

}
