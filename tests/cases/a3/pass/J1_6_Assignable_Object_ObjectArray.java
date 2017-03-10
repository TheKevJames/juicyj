// TYPE_CHECKING,CODE_GENERATION
/**
 * Typecheck:
 * - Type Object[] is assignable to type Object.
 */
public class J1_6_Assignable_Object_ObjectArray {

    public J1_6_Assignable_Object_ObjectArray () {}

    public static int test() {
	Object j = new Object[123];
	return ((Object[])j).length;
    }

}
