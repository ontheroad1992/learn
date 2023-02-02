import java.sql.Date;
import java.util.*;


public class Hello {

  // 每一个程序都需要有一个main函数作为入口
  private static final String Type = null;

  public static void main(String[] args) {
    // 使用 System.out.println 来输出到标准输出
    System.out.println("Hello World1!");
    System.out.println("Integer: " + 10 + " Double: " + 3.14 + " Boolean: " + true);

    // 不自动换行
    System.out.print("Hello ");
    System.out.print("world");

    // 基本类型
    // 字节码类型 8 位
    // (-128 <= fooByte <= 127)
    byte fooByte = 100;

    // 短整型 16 位
    // (-32768 <= fooShort <= 32767)
    short fooShort = 10000;

    // 整形 32 位
    int fooInt = 1;

    // 长整型 64 位
    long fooLong = 100000L;

    // 浮点型 32 位单精度，需要加 f 否则会被当成双精度的
    float fooFloat = 234.5f;

    // 浮点型，双精度
    double fooDouble = 123.4;

    // 布尔类型
    boolean fooBoolean = true || false;

    // 字符类型 - 16位 Unicode编码字符
    char fooChar = 'A';

    // 用 final 可以使一个常量不可更改
    final int HOURS_I_WORK_PER_WEEK = 9001;

    // 字符串
    String fooString =  "My String Is here!";
    System.out.println("fooString: " + fooString);

    // 数组
    int[] intArray = new int[10];
    System.out.println(intArray[1]);
    intArray[1] = 10;
    System.out.println(intArray[1]);

    String[] stringArray = new String[1];
    boolean[] booleanArray = new boolean[100];

    int[] intArray2 = {9000, 123, 2323};

    System.out.println(intArray2[1]);

    // 将字符串转换为整形
    int newInt = Integer.parseInt("10000");
    System.out.println("newInt: " + newInt);

    // 数字转换为字符串
    String newStr = Integer.toString(1000);
    System.out.println("newStr: " + newStr);

    Bicycle trek = new Bicycle(100, 102, 12, "sdsds");
    trek.setCadence(1002);

    System.out.println("trek info: " + trek.toString());

    PennyFarthing penny = new PennyFarthing(100, 123);
    System.out.println("penny info: "  + penny.toString());

    String s1 = "hello world";
    char c = s1.charAt(2);
    System.out.println("c: " + c);

    char[] chars = s1.toCharArray();
    chars[1] = 'a';
    String s2 = new String(chars);
    System.out.println("s2: " + s2);
    System.out.println("boolean: " + s1.equals(s2));

    StringBuilder sb = new StringBuilder("hello world");
    System.out.println("sb: " + sb);
    sb.append("forrest");
    System.out.println("sb: " + sb);

    // 初始化一个存储 String 类型的动态数组
    ArrayList<String> strings = new ArrayList<>();

    // 初始化一个存储 int 类型的动态数组
    ArrayList<Integer> nums = new ArrayList<>();

    int[] arr1 = {10,20,30,40};
    traverse(arr1);
  }

  public static void traverse(int[] arr) {
    for (int i =0; i < arr.length; i++) {
      System.out.println("index: " + i + " value: " + arr[i]);
    }
  }


}

class Bicycle {
  protected int gear;
  public int cadence;
  public int speed;
  String name; // 可以在包内访问

  public Bicycle() {
    gear = 1;
    cadence = 50;
    speed = 5;
    name = "Forrest";
  }

  public Bicycle(int startCadence, int startSpeed, int startGear, String name) {
    this.gear = startGear;
    this.cadence = startCadence;
    this.speed = startSpeed;
    this.name = name;
  }

  public int getCadence() {
    return cadence;
  }

  public void setCadence(int newValue) {
    cadence = newValue;
  }

  @Override
  public String toString() {
    return "gear: " + gear +
    " cadence: " + cadence +
    " speed: " + speed +
    " name: " + name;
  }
}

class PennyFarthing extends Bicycle {
  public PennyFarthing(int startCadence, int startSpeed) {
    super(startCadence, startSpeed, 0, "PennyFarthing");
  }

  @Override
  public void setCadence(int newGear) {
    gear = newGear;
  }
}
