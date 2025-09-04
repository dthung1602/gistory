import "./Header.css";

function Header() {
  return (
    <div className="hero my-8">
      <div className="hero-content text-center">
        <div className="max-w-xxl">
          <h1 className="text-7xl font-bold gradient-primary-content">
            Gistory
          </h1>
          <p className="text-2xl italic py-6 gradient-info-content">
            Create custom commit patterns to display on your GitHub profile
          </p>
        </div>
      </div>
    </div>
  );
}

export default Header;
