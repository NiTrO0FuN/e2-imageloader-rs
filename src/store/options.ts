import { Ref, ref, watch } from "vue";
import { Store } from "@tauri-apps/plugin-store";

type Options = {
  quality: Quality;
  chunkSize: ChunkSize;
  automode: boolean;
};

enum Quality {
  HIGH = "high",
  MEDIUM = "medium",
  LOW = "low",
}

enum ChunkSize {
  LARGE = "large",
  MEDIUM = "medium",
  SMALL = "small",
}

const options: Ref<Options> = ref({
  quality: Quality.HIGH,
  chunkSize: ChunkSize.MEDIUM,
  automode: false,
});

const store = Store.load("settings.json");

// Initial loading
store.then((s) => s.get<Options>("options")).then((opt) => (options.value = opt || options.value));

watch(options, () => store.then((s) => s.set("options", options.value)), { deep: true });

export { Quality, ChunkSize, options };
