package algorithm;

public class algorithm {
  public static void main(String[] args) {
    int[] arrs = {1,2,3,4,5};
    NumArray numArray = new NumArray(arrs);
    System.out.println(numArray.sumRange(0, 3));
  }
}
