
# 常用API


## List<Integer> 和 int[] 互转

```java
List<Integer> list = Arrays.asList(1, 2, 3, 4, 5);
int[] arr = list.stream()
            .mapToInt(Integer::intValue)  // 将 Integer 流转换为 IntStream
            .toArray();

System.out.println(Arrays.toString(arr)); // [1, 2, 3, 4, 5]
```

```java
int[] arr = {1, 2, 3, 4, 5};

List<Integer> list = Arrays.stream(arr)
                    .boxed()        // int → Integer
                    .collect(Collectors.toList());

System.out.println(list); // [1, 2, 3, 4, 5]
```

## List<int[]> 和 int[][]互转

```java
List<int[]> list = new ArrayList<>();
list.add(new int[]{1, 2});
list.add(new int[]{3, 4});

// 转为 int[][]
int[][] array = list.toArray(int[][]::new);

int[][] array = {
    {1, 2},
    {3, 4}
};
List<int[]> list = Arrays.stream(array)
                         .collect(Collectors.toList());
```

## java.util.Arrays

```java
# 初始化List<Integer>
List<Integer> list = Arrays.asList(1, 2, 3, 4, 5);

# int[]数组转String
int[] arr = {1, 2, 3, 4, 5};
String arrStr = Arrays.toString(arr);
```

