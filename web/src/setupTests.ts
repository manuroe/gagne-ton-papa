// jest-dom adds custom jest matchers for asserting on DOM nodes.
// allows you to do things like:
// expect(element).toHaveTextContent(/react/i)
// learn more: https://github.com/testing-library/jest-dom
import '@testing-library/jest-dom';

// jsdom does not implement URL.createObjectURL; stub it for components using Blob URLs
// eslint-disable-next-line @typescript-eslint/no-explicit-any
if (!(globalThis as any).URL) {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	(globalThis as any).URL = {} as any;
}
// eslint-disable-next-line @typescript-eslint/no-explicit-any
if (!('createObjectURL' in (globalThis as any).URL)) {
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	(globalThis as any).URL.createObjectURL = () => 'blob:mock-url';
}
