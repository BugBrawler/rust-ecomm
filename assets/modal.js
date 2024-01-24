(function() {
  const modal = document.querySelector("dialog");
  const showModalButton = document.querySelector("header button");
  const closeModalButton = document.querySelector("dialog button");

  // "Show the dialog" button opens the dialog modally
  showModalButton.addEventListener("click", () => {
    modal.showModal();
  });

  // "Close" button closes the dialog
  closeModalButton.addEventListener("click", () => {
    modal.close();
  });
})();