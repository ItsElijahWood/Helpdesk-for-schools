
const sidebar_light = '/assets/imgs/sidebar-light.svg';
const sidebar_open_light = '/assets/imgs/sidebar-open-light.svg';

const sidebar = document.getElementById('sidebar');
const btn = document.getElementById('sidebarButton');

let open = false;
btn.addEventListener('click', () => {
  open = !open;

  if (open) {
    btn.src = sidebar_open_light;
  } else {
    btn.src = sidebar_light;
  }

  sidebar.classList.toggle('w-74');
});

document.addEventListener('click', (e) => {
  if (
    open &&
    !sidebar.contains(e.target) &&
    !btn.contains(e.target)
  ) {
    open = !open;

    if (open) {
      btn.src = sidebar_open_light;
    } else {
      btn.src = sidebar_light;
    }

    sidebar.classList.toggle('w-74');
  }
});
