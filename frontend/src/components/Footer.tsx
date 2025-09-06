function Footer() {
  return (
    <footer className="footer sm:footer-horizontal bg-base-200 text-base-content p-10">
      <aside>
        <h2 className="gradient-primary-content text-3xl">Gistory</h2>
        <p className="gradient-info-content">Commit patterns on GitHub profile</p>
      </aside>
      <nav>
        <h6 className="footer-title text-info">Link</h6>
        <a className="link link-hover" href="https://github.com/dthung1602/gistory">
          GitHub
        </a>
        <a className="link link-hover" href="https://github.com/dthung1602/gistory/blob/master/LICENSE">
          Licence
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
