const invoke = window.__TAURI__.invoke

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
