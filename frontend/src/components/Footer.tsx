import ferris from "../asset/transparent-ferris.gif";

function Footer() {
  return (
    <footer className="footer sm:footer-horizontal bg-base-200 text-base-content p-10 sm:mt-16 mt-8">
      <aside>
        <h2 className="gradient-primary-content text-3xl">Gistory</h2>
        <p className="gradient-info-content">Commit patterns on GitHub profile</p>
      </aside>
      <nav>
        <h6 className="footer-title text-info">Link</h6>
        <a className="link link-hover" href="https://github.com/dthung1602/gistory">
          GitHub
        </a>
        <a className="link link-hover" href="https://github.com/dthung1602/gistory/issues">
          Report issue
        </a>
        <a className="link link-hover" href="https://github.com/dthung1602/gistory/blob/master/LICENSE">
          Licence
        </a>
      </nav>
      <nav>
        <h6 className="footer-title text-info">Built with</h6>
        <a className="link link-hover" href="https://github.com/tokio-rs/axum">
          Axum
          <a href="https://www.behance.net/gallery/42774743/Rustacean#">
            <img src={ferris} alt="ferris" className="h-6 inline invisible ml-2 ferris" />
          </a>
        </a>
        <a className="link link-hover" href="https://react.dev/">
          React
        </a>
        <a className="link link-hover" href="https://daisyui.com/">
          DaisyUI
        </a>
        <a className="link link-hover" href="https://www.svgrepo.com/ ">
          SVGRepo
        </a>
      </nav>
      <nav>
        <h6 className="footer-title text-info">My other stuff</h6>
        <a className="link link-hover" href="https://dthung1602.github.io/sqss/">
          SQSS
        </a>
        <a className="link link-hover" href="https://manga--bookmark.appspot.com/">
          MangaBookmark
        </a>
        <a className="link link-hover" href="https://dthung1602.github.io/pyobfusinator/">
          PyObfusinator
        </a>
        <a className="link link-hover" href="https://dthung1602.github.io/image2css/">
          Image2CSS
        </a>
      </nav>
    </footer>
  );
}

export default Footer;
