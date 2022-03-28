export default function tryCatch<T, E, C>(t: () => T, c: (e: E) => C) {
	try {
		return t();
	} catch (e: any) {
		return c(e);
	}
}
