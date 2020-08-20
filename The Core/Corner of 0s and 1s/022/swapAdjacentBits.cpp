int swapAdjacentBits(int n) {
  return [](int n){
    int m = 3;
    while(m!=0) {
      if((n&m)!=m && (n&m)!=0) {
        n=n^m;
      }
      m<<=2;
    }
    return n;
  }(n);
}
