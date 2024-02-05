import clsx from 'clsx';
import { FaBook, FaLightbulb } from 'react-icons/fa';
import { FaBoxesStacked, FaGaugeHigh } from 'react-icons/fa6';
import Feature from '../Feature/Feature';
import styles from './Features.module.css';

export default function Features() {
  return (
    <div className={clsx('container', styles.featuresContainer)}>
      <Feature
        icon={<FaLightbulb fontSize={64} />}
        title="Easy to use"
        description="Serwer is the simplest web framework to use, making it perfect for both beginners and experienced developers."
      />
      <Feature
        icon={<FaBoxesStacked fontSize={64} />}
        title="Dependency-free"
        description="Serwer has no dependencies. Everything is built from scratch to suit your needs."
      />
      <Feature
        icon={<FaGaugeHigh fontSize={64} />}
        title="Blazingly fast"
        description="Serwer, despite being so simple, is relatively performant compared to other web frameworks."
      />
      <Feature
        icon={<FaBook fontSize={64} />}
        title="No boilerplate"
        description="Serwer has no boilerplate code, so you can start writing your application right away."
      />
    </div>
  );
}
