int secondRightmostZeroBit(int n) {
  return [](int n) { int p=1; n++; while(!(p&n)) p<<=1; while(p&n) p<<=1; return p; }(n);
}