import CodeBlock from '@theme/CodeBlock';
import Heading from '@theme/Heading';
import clsx from 'clsx';
import { useMemo } from 'react';
import styles from './Side.module.css';

type Props = {
  children: React.ReactNode;
  title: string;
  description: string;
  isLeftToRight: boolean;
};

export default function Side({ children, title, description, isLeftToRight }: Props) {
  const [leftSide, rightSide] = useMemo(() => {
    const codeBlock = (
      <CodeBlock className={styles.codeBlock} language="rust">
        {children}
      </CodeBlock>
    );

    const sideText = (
      <div className={styles.sideText}>
        <Heading as="h2">{title}</Heading>
        <p>{description}</p>
      </div>
    );

    return isLeftToRight ? [codeBlock, sideText] : [sideText, codeBlock];
  }, [isLeftToRight]);

  return (
    <div className={clsx('container', styles.sideContainer, isLeftToRight && styles.sideContainerReverse)}>
      {leftSide} {rightSide}
    </div>
  );
}
