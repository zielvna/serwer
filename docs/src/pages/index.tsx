import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import Features from '../components/Features/Features';
import Hero from '../components/Hero/Hero';
import Sides from '../components/Sides/Sides';

export default function Home() {
  const { siteConfig } = useDocusaurusContext();

  return (
    <Layout description={siteConfig.tagline}>
      <header>
        <Hero />
      </header>
      <main>
        <Features />
        <Sides />
      </main>
    </Layout>
  );
}
