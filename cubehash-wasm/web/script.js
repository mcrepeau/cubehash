// Utility functions
const toHex = (bytes) => Array.from(bytes, b => b.toString(16).padStart(2, "0")).join("");
const bytesToMB = (n) => n / 1_000_000;

// State variables
let lastFileDigest = null;
let lastStringDigest = null;

// Hashing functions
async function hashFileStreamed(file, revision, hashLenBits, onProgress) {
  const hasher = new WasmCubeHash(revision, hashLenBits);
  const reader = file.stream().getReader();
  const total = file.size;
  let processed = 0;

  const t0 = performance.now();
  try {
    while (true) {
      const { value, done } = await reader.read();
      if (done) break;
      hasher.update(value);
      processed += value.length;
      if (onProgress) onProgress(processed / total);
    }
  } finally {
    reader.releaseLock();
  }
  const digest = hasher.finalize();
  const t1 = performance.now();

  return { digest, elapsedMs: t1 - t0 };
}

async function hashString(str, revision, bits) {
  const enc = new TextEncoder().encode(str);
  const hasher = new WasmCubeHash(revision, bits);
  hasher.update(enc);
  return hasher.finalize();
}

// Main function
function main() {
  const $ = (id) => document.getElementById(id);

  // DOM elements
  const fileEl = $("file");
  const fileInfoEl = $("fileInfo");
  const outEl = $("out");
  const timeEl = $("timeMs");
  const mbpsEl = $("mbps");
  const btnFile = $("hashFileBtn");
  const errEl = $("err");
  const copyBtn = $("copyBtn");
  const loader = $("loader");
  const progressBar = $("progressBar");
  const strEl = $("inputStr");
  const revEl = $("revision");
  const bitsEl = $("bits");

  const fileTabBtn = $("fileTab");
  const strTabBtn = $("stringTab");
  const fileTab = $("fileTabContent");
  const strTab = $("stringTabContent");

  // Helper functions
  function showDigest(digest, elapsedMs, fileSize) {
    outEl.textContent = toHex(digest);
    timeEl.value = `${elapsedMs.toFixed(2)} ms`;
    if (fileSize !== undefined) {
      const mb = bytesToMB(fileSize);
      const seconds = elapsedMs / 1000;
      const mbps = seconds > 0 ? (mb / seconds) : 0;
      mbpsEl.value = `${mbps.toFixed(2)} MB/s`;
    } else {
      mbpsEl.value = "–";
    }
  }

  // Tabs
  function switchTab(tab) {
    const perfEl = $("perfInfo");
    if (tab === "file") {
        fileTab.classList.remove("hidden");
        strTab.classList.add("hidden");
        fileTabBtn.classList.add("bg-blue-600","text-white");
        fileTabBtn.classList.remove("bg-gray-200","text-gray-700");
        strTabBtn.classList.add("bg-gray-200","text-gray-700");
        strTabBtn.classList.remove("bg-blue-600","text-white");
        perfEl.classList.remove("hidden");
        if (lastFileDigest) showDigest(lastFileDigest.digest, lastFileDigest.elapsedMs, lastFileDigest.fileSize);
        else outEl.textContent = "";
    } else {
        strTab.classList.remove("hidden");
        fileTab.classList.add("hidden");
        strTabBtn.classList.add("bg-blue-600","text-white");
        strTabBtn.classList.remove("bg-gray-200","text-gray-700");
        fileTabBtn.classList.add("bg-gray-200","text-gray-700");
        fileTabBtn.classList.remove("bg-blue-600","text-white");
        perfEl.classList.add("hidden");
        if (lastStringDigest) showDigest(lastStringDigest.digest, lastStringDigest.elapsedMs);
        else outEl.textContent = "";
    }
  }

  // Event listeners
  fileTabBtn.onclick = () => switchTab("file");
  strTabBtn.onclick = () => switchTab("string");

  // File selection
  function updateFileBtnState() {
    const hasFile = !!fileEl.files?.length;
    btnFile.disabled = !hasFile;
    btnFile.classList.toggle("opacity-50", !hasFile);
    btnFile.classList.toggle("cursor-not-allowed", !hasFile);
  }

  // Call once on page load
  updateFileBtnState();

  // Update when file selection changes
  fileEl.addEventListener("change", () => {
    const f = fileEl.files?.[0];
    fileInfoEl.textContent = f ? `${f.name} (${(f.size/1024).toFixed(1)} KB)` : "";
    updateFileBtnState();
  });

  // Hash file
  btnFile.addEventListener("click", async () => {
    errEl.textContent = "";
    outEl.textContent = "";
    timeEl.value = "–"; mbpsEl.value = "–";

    const f = fileEl.files?.[0];
    if (!f) { errEl.textContent = "Pick a file first."; return; }

    loader.classList.remove("hidden");
    progressBar.classList.remove("hidden");
    progressBar.value = 0;

    try {
      const { digest, elapsedMs } = await hashFileStreamed(
        f, Number(revEl.value), Number(bitsEl.value),
        (p) => progressBar.value = p * 100
      );
      lastFileDigest = { digest, elapsedMs, fileSize: f.size };
      showDigest(digest, elapsedMs, f.size);
    } catch (e) {
      errEl.textContent = `Error: ${e}`;
    } finally {
      loader.classList.add("hidden");
      progressBar.classList.add("hidden");
    }
  });

  // Hash string on input or param change
  async function updateStringHash() {
    const str = strEl.value;
    try {
        const digest = await hashString(str, Number(revEl.value), Number(bitsEl.value));
        lastStringDigest = { digest, elapsedMs: 0 };
        showDigest(digest, 0);
    } catch (e) {
        errEl.textContent = `Error: ${e}`;
    }
  }

  strEl.addEventListener("input", updateStringHash);
  revEl.addEventListener("change", updateStringHash);
  bitsEl.addEventListener("change", updateStringHash);

  // Copy
  copyBtn.addEventListener("click", async () => {
    if (!outEl.textContent) return;
    try {
        await navigator.clipboard.writeText(outEl.textContent);
        const msg = $("copyMsg");
        msg.classList.remove("hidden");
        setTimeout(() => msg.classList.add("hidden"), 1500);
    } catch {}
  });

  switchTab("file");
}