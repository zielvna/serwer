import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Heading from '@theme/Heading';
import clsx from 'clsx';
import styles from './Hero.module.css';

export default function Hero() {
  const { siteConfig } = useDocusaurusContext();

  return (
    <div className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className="hero__title">
          {siteConfig.title}
        </Heading>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to={(siteConfig.themeConfig.footer as any).links[0].items[0].to}
          >
            Get started
          </Link>
        </div>
      </div>
    </div>
  );
}
