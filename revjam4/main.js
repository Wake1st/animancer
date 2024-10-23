const menuToggle = document.querySelector("#toggle_menu");
const ambientToggle = document.querySelector("#toggle_ambient");
const battleToggle = document.querySelector("#toggle_battle");

const menuAudio = document.querySelector("#audio_menu");
const ambientAudio = document.querySelector("#audio_ambient");
const battleAudio = document.querySelector("#audio_battle");

function toggleMenu() {
  console.log("toggleMenu");

  ambientAudio.pause();
  battleAudio.pause();

  ambientToggle.checked = false;
  battleToggle.checked = false;
  
  if (menuToggle) {
    menuAudio.play();
  } else {
    menuAudio.pause();
  }
}

function toggleAmbient() {
  console.log("toggleAmbient");

  menuAudio.pause();
  battleAudio.pause();

  menuToggle.checked = false;
  battleToggle.checked = false;

  if (ambientToggle) {
    ambientAudio.play();
  } else {
    ambientAudio.pause();
  }
}

function toggleBattle() {
  console.log("toggleBattle");

  menuAudio.pause();
  ambientAudio.pause();

  menuToggle.checked = false;
  ambientToggle.checked = false;

  if (battleToggle) {
    battleAudio.play();
  } else {
    battleAudio.pause();
  }
}

menuToggle.checked = true;
battleAudio.volume = 0.5;