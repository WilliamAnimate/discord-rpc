const invoke = window.__TAURI__.invoke

// function setRPC() {
// invoke("set_client_id", {x: "1173464118787846165"});
// invoke("set_details", {x: "hey!!11!"});
// invoke("set_state", {x: "currently testing (fr)"});
// if (document.getElementById("timestamp_enabled").checked) {
// 	invoke("set_timestamp_enabled", {x: true});
// }
// invoke("set_button_one_text", {x: "website"});
// invoke("set_button_one_link", {x: "https://williamanimate.github.io"});
// invoke("set_button_two_text", {x: "github"});
// invoke("set_button_two_link", {x: "https://github.com/williamanimate"});
// invoke("set_large_image_asset_name", {x: "beetlefr"});
// invoke("set_large_image_asset_text", {x: "damn i'd smash fr (real)"});
// invoke("set_small_image_asset_name", {x: "niko"});
// invoke("set_small_image_asset_text", {x: "niko oneshot!!11!!!!!"});
// invoke("rich_presence_callback");
// }

// remove styling on paste
document.addEventListener('paste', (e) => {
	e.preventDefault();
	// FIXME: execCommand deprecated 💀
	document.execCommand('insertText', false, (e.clipboardData || window.Clipboard).getData('text/plain'));
});

function getTextcontentByIdName(elementName) {
	return document.getElementById(elementName).textContent;
}

console.log(invoke("check_if_rpc_is_up"));

function setRPC() {
	// TODO: fix indentation hell
	invoke("check_if_rpc_is_up").then((r) => {
		if (r == false) {
			invoke("set_client_id", {x: getTextcontentByIdName("clientid")});
			invoke("set_details", {x: getTextcontentByIdName("details")});
			invoke("set_state", {x: getTextcontentByIdName("state")});
			if (document.getElementById("timestamp_enabled").checked) {
				invoke("set_timestamp_enabled", {x: true});
			}
			invoke("set_button_one_text", {x: getTextcontentByIdName("btn1txt")});
			invoke("set_button_one_link", {x: getTextcontentByIdName("btn1lnk")});
			invoke("set_button_two_text", {x: getTextcontentByIdName("btn2txt")});
			invoke("set_button_two_link", {x: getTextcontentByIdName("btn2lnk")});
			invoke("set_large_image_asset_name", {x: getTextcontentByIdName("largeimagename")});
			invoke("set_large_image_asset_text", {x: getTextcontentByIdName("largeimagetext")});
			invoke("set_small_image_asset_name", {x: getTextcontentByIdName("smallimagename")});
			invoke("set_small_image_asset_text", {x: getTextcontentByIdName("smallimagetext")});

			invoke("rich_presence_callback");
		}
	});
}

function stopRPC() {
	invoke("rpc_stop_thread");
}

// function a() {
// 	console.log(document.getElementById("details").textContent)
// }