
// From lib.webworker.d.ts
// file:///opt/visual-studio-code/resources/app/extensions/node_modules/typescript/lib/lib.webworker.d.ts

type EndingType = "native" | "transparent";

interface BlobPropertyBag {
  endings?: EndingType;
  type?: string;
}

interface FilePropertyBag extends BlobPropertyBag {
  lastModified?: number;
}

/**
 * Provides information about files and allows JavaScript in a web page to access their content.
 *
 * [MDN Reference](https://developer.mozilla.org/docs/Web/API/File)
 */
interface File extends Blob {
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/File/lastModified) */
  readonly lastModified: number;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/File/name) */
  readonly name: string;
  /** [MDN Reference](https://developer.mozilla.org/docs/Web/API/File/webkitRelativePath) */
  readonly webkitRelativePath: string;
}

declare var File: {
  prototype: File;
  new(fileBits: BlobPart[], fileName: string, options?: FilePropertyBag): File;
};

type FormDataEntryValue = File | string;

/** [MDN Reference](https://developer.mozilla.org/docs/Web/API/fetch) */
// declare function fetch(input: RequestInfo | URL, init?: RequestInit): Promise<Response>;
declare function fetch(input: string, init?: RequestInit): Promise<Response>;
