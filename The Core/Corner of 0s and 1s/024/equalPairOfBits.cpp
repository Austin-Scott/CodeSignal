int equalPairOfBits(int n, int m) {
  return [&](){
    int mask=1;
    while((n&mask)!=(m&mask)) {
      mask<<=1;
    }
    return mask;
  }();
}