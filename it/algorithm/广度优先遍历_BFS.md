
# 题目

## 279.完全平方数
https://leetcode.cn/problems/perfect-squares/description/?envType=problem-list-v2&envId=6uaxYMyj

题解：
https://algo.itcharge.cn/solutions/0200-0299/perfect-squares/#%E8%A7%A3%E9%A2%98%E6%80%9D%E8%B7%AF

1.	定义 visited为标记访问节点的 set 集合变量，避免重复计算。定义 queue为存放节点的队列。使用 count表示为树的最小深度，也就是和为 n 的完全平方数的最小数量。
2.	首先，我们将 n 标记为已访问，即 visited.add(n)。并将其加入队列 queue中，即 queue.append(n)。
3.	令 count加 11，表示最小深度加 11。然后依次将队列中的节点值取出。
4.	对于取出的节点值 value，遍历可能出现的平方数（即遍历 [1,value+1] 中的数）。
5.	每次从当前节点值减去一个平方数，并将减完的数加入队列。
1.	如果此时的数等于 0，则满足题意，返回当前树的最小深度。
2.	如果此时的数不等于 0，则将其加入队列，继续查找。

```java
class Solution {
    public int numSquares(int n) {
        Queue<Integer> queue = new LinkedList<>();
        Set<Integer> visited = new HashSet<>();
        queue.offer(n);
        visited.add(n);
        int res = 0;
        while (!queue.isEmpty()) {
            int len = queue.size();
            res++;
            for (int i = 0; i < len; i++) {
                int val = queue.poll();
                for (int j = 1; j <= (int)Math.sqrt(val)+1; j++) {
                    int x = val - j*j;
                    if (x == 0) {
                         return res;
                    }
                    if (!visited.contains(x)) {
                        queue.offer(x);
                        visited.add(x);
                    }
                }
            }
        }
        return res;
    }
}
```