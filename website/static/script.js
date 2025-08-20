let isAbout = true;
let navLink = document.getElementById("nav-privacy");

const saveId = document.getElementById("saveId");
const loading = document.getElementById("loading");
const saveSection = document.getElementById("save");
const links = document.querySelectorAll(".nav-link");
const sections = document.querySelectorAll(".section");
const searchInput = document.getElementById("search-input");
const searchButton = document.getElementById("search-button");
const loadingMessage = document.getElementById("loading-message");
const scrollableContainer = document.querySelector(".scrollable-container");

console.log("%cFORGOTTEN?: Show some love star the github repo https://github.com/devfemibadmus/mediascraper.", "color: #00faff; font-size: 16px;");
console.log(
	"%cTHE APP?: Show some love star the APP github repo https://github.com/devfemibadmus/mediasaver get yours on playstore.",
	"color: #00faff; font-size: 16px;"
);

searchButton.addEventListener("click", function (event) {
	event.preventDefault();
	const url = searchInput.value;
	var payload = { url: url, cut: true };

	sections.forEach((section) => {
		section.classList.remove("active");
	});
	saveSection.classList.add("active");

	loadingMessage.textContent = "Loading";
	loadingMessage.style.color = "grey";
	loading.style.display = "inline";
	loading.style.color = "grey";

	fetch("/api/", {
		method: "POST",
		headers: { "Content-Type": "application/json" },
		body: JSON.stringify(payload),
	})
		.then((response) => response.json())
		.then((response) => {
			console.log(response);
			loading.style.display = "none";
			loadingMessage.textContent = response.message;
			if (response.error) {
				loadingMessage.style.color = "red";
			} else if (response.success) {
				loadingMessage.style.color = "#00faff";
				const data = response.data;
				console.log(data);
				if (data.platform == "tiktok") {
					const tiktokContentManager = new TikTokContentManager(scrollableContainer, saveId);
					tiktokContentManager.setContent(data.content);
					tiktokContentManager.setMusic(data.music);
					tiktokContentManager.setAuthor(data.author);
					tiktokContentManager.setMedia(data);
					console.log(data.platform);
				} else if (data.platform == "instagram") {
					const instagramContentManager = new InstagramContentManager(scrollableContainer, saveId);
					instagramContentManager.setContent(data.content);
					instagramContentManager.setAuthor(data.author);
					instagramContentManager.setMedia(data.media);
					console.log(data.platform);
				} else if (data.platform == "facebook") {
					const facebookContentManager = new FacebookContentManager(scrollableContainer, saveId);
					facebookContentManager.setContent(data.content);
					facebookContentManager.setAuthor(data.author);
					facebookContentManager.setMedia(data.media);
					console.log(data.platform);
				}
			}
		})
		.catch((error) => {
			console.error("Error:", error);
		});
});

class TikTokContentManager {
	constructor(container, saveId) {
		this.container = container;
		this.saveId = saveId;
	}
	formatValue(v) {
		return typeof v === "number" ? (v >= 1e6 ? (v / 1e6).toFixed(1) + "m" : v >= 1e3 ? (v / 1e3).toFixed(1) + "k" : v) : v || "N/A";
	}
	createContainer(obj, exclude = [], titleText = "Item", mediaSrc = null, mediaType = "img", downloadAttr = null) {
		const div = document.createElement("div");
		const title = document.createElement("p");
		div.className = "container post";
		title.className = "title";
		title.innerHTML = `<p class='key'>${titleText}:</p>`;

		for (const key in obj) {
			if (!exclude.includes(key) && obj.hasOwnProperty(key)) {
				const spanKey = document.createElement("span");
				spanKey.innerHTML = downloadAttr ? `<a href='${downloadAttr}' download class='key'>${key}: </a>` : key + ": ";
				const spanValue = document.createElement("span");
				spanValue.textContent = this.formatValue(obj[key]) + " ";
				title.appendChild(spanKey);
				title.appendChild(spanValue);
			}
		}

		div.append(title);

		if (mediaSrc) {
			const media = mediaType === "video" ? document.createElement("video") : document.createElement("img");
			media.className = "productimg";
			media.crossOrigin = "anonymous";
			if (mediaType === "video") media.controls = true;
			media.src = mediaSrc;
			div.insertBefore(media, title);
		}

		this.container.insertBefore(div, this.saveId.nextSibling);
		return div;
	}
	setContent(content) {
		this.createContainer(content, ["cover", "id"], "Post", content.cover, "img", content.cover);
	}
	setAuthor(author) {
		this.createContainer(author, ["image", "id"], "Author", author.image);
	}
	setMusic(music) {
		this.createContainer(music, ["src", "cover"], "Music", music.cover, "img", music.src);
	}
	setMedia(datas) {
		const mediaList = datas.videos ?? datas.images;
		mediaList.forEach((m) => {
			for (const key in m) {
				if (m.hasOwnProperty(key)) {
					const isVideo = datas.is_video;
					const src = "https://api.cors.lol/?url=" + encodeURIComponent(m[key]["address"]);
					this.createContainer(m[key], ["address", "is_video", "cover", "id"], key, src, isVideo ? "video" : "img");
				}
			}
		});
	}
}

class InstagramContentManager {
	constructor(container, saveId) {
		this.container = container;
		this.saveId = saveId;
	}
	formatValue(v) {
		return typeof v === "number" ? (v >= 1e6 ? (v / 1e6).toFixed(1) + "m" : v >= 1e3 ? (v / 1e3).toFixed(1) + "k" : v) : v;
	}
	createContainer(obj, excludeKeys = [], mediaKey = null) {
		const div = document.createElement("div");
		const title = document.createElement("p");
		div.className = "container post";
		title.className = "title";
		title.innerHTML = `<p class='key'>${mediaKey || "Item"}:</p>`;

		for (const key in obj) {
			if (!excludeKeys.includes(key) && obj.hasOwnProperty(key)) {
				const spanKey = document.createElement("span");
				spanKey.className = "key";
				spanKey.textContent = key + ": ";

				const spanValue = document.createElement("span");
				spanValue.className = key;
				spanValue.textContent = this.formatValue(obj[key]) + " ";

				title.appendChild(spanKey);
				title.appendChild(spanValue);
			}
		}

		div.append(title);
		this.container.insertBefore(div, this.saveId.nextSibling);
		return div;
	}
	setContent(content) {
		const div = this.createContainer(content, ["cover", "id"], "Post");
		const img = document.createElement("img");
		img.className = "productimg";
		img.referrerPolicy = "no-referrer";
		img.crossOrigin = "anonymous";
		img.src = "https://api.cors.lol/?url=" + encodeURIComponent(content.cover);
		div.insertBefore(img, div.firstChild);
	}
	setAuthor(author) {
		const div = this.createContainer(author, ["image", "id"], "Author");
		const img = document.createElement("img");
		img.className = "productimg";
		img.referrerPolicy = "no-referrer";
		img.crossOrigin = "anonymous";
		img.src = "https://api.cors.lol/?url=" + encodeURIComponent(author.image);
		div.insertBefore(img, div.firstChild);
	}
	setMedia(media) {
		media.forEach((m) => {
			const div = this.createContainer(m, ["is_video", "address", "cover", "id"]);
			const el = m.is_video ? document.createElement("video") : document.createElement("img");
			el.className = "productimg";
			el.referrerPolicy = "no-referrer";
			el.crossOrigin = "anonymous";
			el.controls = true;
			el.src = "https://api.cors.lol/?url=" + encodeURIComponent(m.address);
			div.insertBefore(el, div.firstChild);
		});
	}
}

class FacebookContentManager {
	constructor(container, saveId) {
		this.container = container;
		this.saveId = saveId;
	}
	formatValue(v) {
		return typeof v === "number" ? (v >= 1e6 ? (v / 1e6).toFixed(1) + "m" : v >= 1e3 ? (v / 1e3).toFixed(1) + "k" : v) : v;
	}
	createContainer(obj, exclude = [], titleText = "Item", mediaSrc = null, mediaType = "img", downloadAttr = null) {
		const div = document.createElement("div");
		const title = document.createElement("p");
		div.className = "container post";
		title.className = "title";
		title.innerHTML = `<p class='key'>${titleText}:</p>`;

		for (const key in obj) {
			if (!exclude.includes(key) && obj.hasOwnProperty(key)) {
				const spanKey = document.createElement("span");
				spanKey.innerHTML = downloadAttr ? `<a href='${downloadAttr}' download class='key'>${key}: </a>` : key + ": ";
				const spanValue = document.createElement("span");
				spanValue.textContent = this.formatValue(obj[key]) + " ";
				title.appendChild(spanKey);
				title.appendChild(spanValue);
			}
		}

		div.append(title);

		if (mediaSrc) {
			const media = mediaType === "video" ? document.createElement("video") : document.createElement("img");
			media.className = "productimg";
			media.referrerPolicy = "no-referrer";
			media.crossOrigin = "anonymous";
			if (mediaType === "video") media.controls = true;
			media.src = mediaSrc;
			div.insertBefore(media, title);
		}

		this.container.insertBefore(div, this.saveId.nextSibling);
		return div;
	}
	setContent(content) {
		this.createContainer(
			content,
			["cover", "id"],
			"Post",
			"https://api.cors.lol/?url=" + encodeURIComponent(content.cover),
			"img",
			content.cover
		);
	}
	setAuthor(author) {
		this.createContainer(author, ["image"], "Author", "/static/facebook.png");
	}
	setMedia(mediaList) {
		mediaList.forEach((m) => {
			const isVideo = m.is_video;
			const src = "https://api.cors.lol/?url=" + encodeURIComponent(m.address);
			this.createContainer(m, ["is_video", "address", "cover"], m.id, src, isVideo ? "video" : "img", m.address);
		});
	}
}

links.forEach((link) => {
	link.addEventListener("click", function (event) {
		event.preventDefault();
		const targetId = this.getAttribute("href").substring(1);
		const targetElement = document.getElementById(targetId);

		if (targetElement) {
			sections.forEach((container) => {
				container.classList.remove("active");
			});

			targetElement.classList.add("active");
		}
	});
});

document.addEventListener("DOMContentLoaded", function () {
	const currentHash = window.location.hash.substring(1);
	const targetElement = document.getElementById(currentHash);

	if (targetElement) {
		sections.forEach((container) => {
			container.classList.remove("active");
		});

		targetElement.classList.add("active");
	}
});

setInterval(() => {
	navLink.classList.remove("visible");
	navLink.classList.add("hidden");

	setTimeout(() => {
		navLink.textContent = isAbout ? "About" : "Privacy";
		navLink.classList.remove("hidden");
		navLink.classList.add("visible");
		isAbout = !isAbout;
	}, 500);
}, 8000);

document.addEventListener("click", function (e) {
	if (e.target.tagName === "A" && e.target.classList.contains("key")) {
		e.preventDefault();
		loadingMessage.textContent = "Loading";
		loadingMessage.style.color = "red";
		loading.style.display = "inline";
		loading.style.color = "red";
		fetch(e.target.href)
			.then((res) => res.blob())
			.then((blob) => {
				const url = URL.createObjectURL(blob);
				const a = document.createElement("a");
				a.href = url;
				a.download = e.target.getAttribute("download");
				document.body.appendChild(a);
				a.click();
				a.remove();
				URL.revokeObjectURL(url);
				loadingMessage.textContent = "Downloading";
				loading.style.display = "none";
			})
			.catch(() => {
				loadingMessage.textContent = "Downloading";
				loading.style.display = "none";
			});
	}
});
