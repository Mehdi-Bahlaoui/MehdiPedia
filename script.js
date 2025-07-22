
  document.addEventListener("DOMContentLoaded", function () {
    const expandableItems = document.querySelectorAll(".toc-list li.expandable");

    expandableItems.forEach(item => {
      item.addEventListener("click", function (e) {
        // Prevent click from navigating if <a> is inside
        if (e.target.tagName.toLowerCase() === "a") return;

        item.classList.toggle("expanded");
        item.classList.toggle("collapsed");
      });
    });
  });

