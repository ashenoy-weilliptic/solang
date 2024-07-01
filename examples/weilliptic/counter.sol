contract counter {
    uint64 public count = 0;

    function increment() public returns (uint64){
        count += 1;
        return count;
    }

    function decrement() public returns (uint64){
        count -= 1;
        return count;
    }

    function double() public returns (uint64) {
        count *= 2;
        return count;
    }

    function reset() public returns (uint64) {
        count = 0;
        return count;
    }
}
