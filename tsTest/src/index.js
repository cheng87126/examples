function* f() {
	yield 1;
	yield* [2, 3];
}