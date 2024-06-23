# up_or_down.rs

Inspired by an interview question addressed in a [Youtube video by Low Level Learning.](https://www.youtube.com/watch?v=V2h_hJ5MSpY)

### Prompt

```
Write a program that can compute if the stack grows up or down for the computer architecture you are working on.
```

### Low Level Learning's Solution in C

``` c
bool upordown(int *other) {
    int x;
    if (!other) {
        return upordown(&x);
    } else {
        return &x > &other;
    }
}

int main() {
    printf("%s\n", upordown(NULL) ? "Up" : "Down");
    return 0;
}
```
