int killKthBit(int n, int k) {
  return ~(1<<k-1)&n;
}